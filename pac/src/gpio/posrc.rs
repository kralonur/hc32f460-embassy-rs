#[doc = "Register `POSRC` reader"]
pub type R = crate::R<PosrcSpec>;
#[doc = "Register `POSRC` writer"]
pub type W = crate::W<PosrcSpec>;
#[doc = "Field `POS13` reader - desc POS13"]
pub type Pos13R = crate::BitReader;
#[doc = "Field `POS13` writer - desc POS13"]
pub type Pos13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS14` reader - desc POS14"]
pub type Pos14R = crate::BitReader;
#[doc = "Field `POS14` writer - desc POS14"]
pub type Pos14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS15` reader - desc POS15"]
pub type Pos15R = crate::BitReader;
#[doc = "Field `POS15` writer - desc POS15"]
pub type Pos15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - desc POS13"]
    #[inline(always)]
    pub fn pos13(&self) -> Pos13R {
        Pos13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POS14"]
    #[inline(always)]
    pub fn pos14(&self) -> Pos14R {
        Pos14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POS15"]
    #[inline(always)]
    pub fn pos15(&self) -> Pos15R {
        Pos15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc POS13"]
    #[inline(always)]
    pub fn pos13(&mut self) -> Pos13W<'_, PosrcSpec> {
        Pos13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POS14"]
    #[inline(always)]
    pub fn pos14(&mut self) -> Pos14W<'_, PosrcSpec> {
        Pos14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POS15"]
    #[inline(always)]
    pub fn pos15(&mut self) -> Pos15W<'_, PosrcSpec> {
        Pos15W::new(self, 15)
    }
}
#[doc = "desc POSRC\n\nYou can [`read`](crate::Reg::read) this register and get [`posrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PosrcSpec;
impl crate::RegisterSpec for PosrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`posrc::R`](R) reader structure"]
impl crate::Readable for PosrcSpec {}
#[doc = "`write(|w| ..)` method takes [`posrc::W`](W) writer structure"]
impl crate::Writable for PosrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POSRC to value 0"]
impl crate::Resettable for PosrcSpec {}
