#[doc = "Register `POTRH` reader"]
pub type R = crate::R<PotrhSpec>;
#[doc = "Register `POTRH` writer"]
pub type W = crate::W<PotrhSpec>;
#[doc = "Field `POT00` reader - desc POT00"]
pub type Pot00R = crate::BitReader;
#[doc = "Field `POT00` writer - desc POT00"]
pub type Pot00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT01` reader - desc POT01"]
pub type Pot01R = crate::BitReader;
#[doc = "Field `POT01` writer - desc POT01"]
pub type Pot01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT02` reader - desc POT02"]
pub type Pot02R = crate::BitReader;
#[doc = "Field `POT02` writer - desc POT02"]
pub type Pot02W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POT00"]
    #[inline(always)]
    pub fn pot00(&self) -> Pot00R {
        Pot00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POT01"]
    #[inline(always)]
    pub fn pot01(&self) -> Pot01R {
        Pot01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POT02"]
    #[inline(always)]
    pub fn pot02(&self) -> Pot02R {
        Pot02R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POT00"]
    #[inline(always)]
    pub fn pot00(&mut self) -> Pot00W<'_, PotrhSpec> {
        Pot00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POT01"]
    #[inline(always)]
    pub fn pot01(&mut self) -> Pot01W<'_, PotrhSpec> {
        Pot01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POT02"]
    #[inline(always)]
    pub fn pot02(&mut self) -> Pot02W<'_, PotrhSpec> {
        Pot02W::new(self, 2)
    }
}
#[doc = "desc POTRH\n\nYou can [`read`](crate::Reg::read) this register and get [`potrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`potrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PotrhSpec;
impl crate::RegisterSpec for PotrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`potrh::R`](R) reader structure"]
impl crate::Readable for PotrhSpec {}
#[doc = "`write(|w| ..)` method takes [`potrh::W`](W) writer structure"]
impl crate::Writable for PotrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POTRH to value 0"]
impl crate::Resettable for PotrhSpec {}
