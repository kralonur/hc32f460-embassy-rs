#[doc = "Register `TCMD` reader"]
pub type R = crate::R<TcmdSpec>;
#[doc = "Register `TCMD` writer"]
pub type W = crate::W<TcmdSpec>;
#[doc = "Field `TSA` reader - desc TSA"]
pub type TsaR = crate::BitReader;
#[doc = "Field `TSA` writer - desc TSA"]
pub type TsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSALL` reader - desc TSALL"]
pub type TsallR = crate::BitReader;
#[doc = "Field `TSALL` writer - desc TSALL"]
pub type TsallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSONE` reader - desc TSONE"]
pub type TsoneR = crate::BitReader;
#[doc = "Field `TSONE` writer - desc TSONE"]
pub type TsoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPA` reader - desc TPA"]
pub type TpaR = crate::BitReader;
#[doc = "Field `TPA` writer - desc TPA"]
pub type TpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPE` reader - desc TPE"]
pub type TpeR = crate::BitReader;
#[doc = "Field `TPE` writer - desc TPE"]
pub type TpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOM` reader - desc LOM"]
pub type LomR = crate::BitReader;
#[doc = "Field `LOM` writer - desc LOM"]
pub type LomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBSEL` reader - desc TBSEL"]
pub type TbselR = crate::BitReader;
#[doc = "Field `TBSEL` writer - desc TBSEL"]
pub type TbselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TSA"]
    #[inline(always)]
    pub fn tsa(&self) -> TsaR {
        TsaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TSALL"]
    #[inline(always)]
    pub fn tsall(&self) -> TsallR {
        TsallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TSONE"]
    #[inline(always)]
    pub fn tsone(&self) -> TsoneR {
        TsoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TPA"]
    #[inline(always)]
    pub fn tpa(&self) -> TpaR {
        TpaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TPE"]
    #[inline(always)]
    pub fn tpe(&self) -> TpeR {
        TpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LOM"]
    #[inline(always)]
    pub fn lom(&self) -> LomR {
        LomR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TBSEL"]
    #[inline(always)]
    pub fn tbsel(&self) -> TbselR {
        TbselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TSA"]
    #[inline(always)]
    pub fn tsa(&mut self) -> TsaW<'_, TcmdSpec> {
        TsaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TSALL"]
    #[inline(always)]
    pub fn tsall(&mut self) -> TsallW<'_, TcmdSpec> {
        TsallW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TSONE"]
    #[inline(always)]
    pub fn tsone(&mut self) -> TsoneW<'_, TcmdSpec> {
        TsoneW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TPA"]
    #[inline(always)]
    pub fn tpa(&mut self) -> TpaW<'_, TcmdSpec> {
        TpaW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TPE"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TpeW<'_, TcmdSpec> {
        TpeW::new(self, 4)
    }
    #[doc = "Bit 6 - desc LOM"]
    #[inline(always)]
    pub fn lom(&mut self) -> LomW<'_, TcmdSpec> {
        LomW::new(self, 6)
    }
    #[doc = "Bit 7 - desc TBSEL"]
    #[inline(always)]
    pub fn tbsel(&mut self) -> TbselW<'_, TcmdSpec> {
        TbselW::new(self, 7)
    }
}
#[doc = "desc TCMD\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmdSpec;
impl crate::RegisterSpec for TcmdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcmd::R`](R) reader structure"]
impl crate::Readable for TcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`tcmd::W`](W) writer structure"]
impl crate::Writable for TcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCMD to value 0"]
impl crate::Resettable for TcmdSpec {}
