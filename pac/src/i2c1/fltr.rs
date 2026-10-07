#[doc = "Register `FLTR` reader"]
pub type R = crate::R<FltrSpec>;
#[doc = "Register `FLTR` writer"]
pub type W = crate::W<FltrSpec>;
#[doc = "Field `DNF` reader - desc DNF"]
pub type DnfR = crate::FieldReader;
#[doc = "Field `DNF` writer - desc DNF"]
pub type DnfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DNFEN` reader - desc DNFEN"]
pub type DnfenR = crate::BitReader;
#[doc = "Field `DNFEN` writer - desc DNFEN"]
pub type DnfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFEN` reader - desc ANFEN"]
pub type AnfenR = crate::BitReader;
#[doc = "Field `ANFEN` writer - desc ANFEN"]
pub type AnfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc DNF"]
    #[inline(always)]
    pub fn dnf(&self) -> DnfR {
        DnfR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - desc DNFEN"]
    #[inline(always)]
    pub fn dnfen(&self) -> DnfenR {
        DnfenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ANFEN"]
    #[inline(always)]
    pub fn anfen(&self) -> AnfenR {
        AnfenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc DNF"]
    #[inline(always)]
    pub fn dnf(&mut self) -> DnfW<'_, FltrSpec> {
        DnfW::new(self, 0)
    }
    #[doc = "Bit 4 - desc DNFEN"]
    #[inline(always)]
    pub fn dnfen(&mut self) -> DnfenW<'_, FltrSpec> {
        DnfenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc ANFEN"]
    #[inline(always)]
    pub fn anfen(&mut self) -> AnfenW<'_, FltrSpec> {
        AnfenW::new(self, 5)
    }
}
#[doc = "desc FLTR\n\nYou can [`read`](crate::Reg::read) this register and get [`fltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltrSpec;
impl crate::RegisterSpec for FltrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltr::R`](R) reader structure"]
impl crate::Readable for FltrSpec {}
#[doc = "`write(|w| ..)` method takes [`fltr::W`](W) writer structure"]
impl crate::Writable for FltrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTR to value 0x10"]
impl crate::Resettable for FltrSpec {
    const RESET_VALUE: u32 = 0x10;
}
